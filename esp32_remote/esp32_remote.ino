#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include "driver/touch_sensor.h"

#define SERVICE_UUID        "4fafc201-1fb5-459e-8fcc-c5c9c331914b"
#define CHARACTERISTIC_UUID "beb5483e-36e1-4688-b7f5-ea07361b26a8"

const int touchPins[] = {T3, T4, T5, T6, T7, T8, T9};
const int touchThreshold = 90;
const int codes[] = {100, 150, 200, 250, 300, 350, 400};
const int numSensors = sizeof(touchPins) / sizeof(touchPins[0]);

const int powerButtonPin = 0; // GPIO 0 for Power Button
const int ledPin = 2; // GPIO 2 for LED

bool sensorActive = false;
bool deviceConnected = false;
bool oldDeviceConnected = false;
bool powerOn = true; // Start with power on

BLEServer *pServer = NULL;
BLECharacteristic *pCharacteristic = NULL;

class MyServerCallbacks: public BLEServerCallbacks {
    void onConnect(BLEServer* pServer) {
        deviceConnected = true;
        digitalWrite(ledPin, LOW);
    };

    void onDisconnect(BLEServer* pServer) {
        deviceConnected = false;
    }
};

void setup() {
    Serial.begin(115200);
    pinMode(powerButtonPin, INPUT_PULLUP); // Power button with pull-up resistor
    pinMode(ledPin, OUTPUT);

    BLEDevice::init("ESP32_Remote");
    pServer = BLEDevice::createServer();
    pServer->setCallbacks(new MyServerCallbacks());

    BLEService *pService = pServer->createService(SERVICE_UUID);
    pCharacteristic = pService->createCharacteristic(
        CHARACTERISTIC_UUID,
        BLECharacteristic::PROPERTY_READ | BLECharacteristic::PROPERTY_WRITE | BLECharacteristic::PROPERTY_NOTIFY
    );
    pCharacteristic->addDescriptor(new BLE2902());
    pCharacteristic->setValue("0");
    pService->start();

    BLEAdvertising *pAdvertising = BLEDevice::getAdvertising();
    pAdvertising->addServiceUUID(SERVICE_UUID);
    pAdvertising->setScanResponse(false);
    pAdvertising->setMinPreferred(0x06);
    pAdvertising->setMinPreferred(0x12);
    BLEDevice::startAdvertising();
    Serial.println("Characteristic defined! Now you can read it in your phone!");
}

void loop() {
    // Power button logic
    if (digitalRead(powerButtonPin) == LOW) { // Button pressed
        delay(50); // Debounce
        if (digitalRead(powerButtonPin) == LOW) {
            powerOn = !powerOn;
            if (!powerOn) {
                BLEDevice::deinit(); // Turn off BLE
                digitalWrite(ledPin, LOW); // Turn off LED
                Serial.println("Power Off");
            } else {
                BLEDevice::init("ESP32_Remote"); // Turn on BLE
                pServer->startAdvertising();
                Serial.println("Power On");
            }
            while (digitalRead(powerButtonPin) == LOW); // Wait for release
        }
    }

    if (powerOn) {
        if (!sensorActive) {
            for (int i = 0; i < numSensors; i++) {
                if (touchRead(touchPins[i]) < touchThreshold) {
                    sensorActive = true;
                    String message = String(codes[i]);
                    pCharacteristic->setValue(message.c_str());
                    pCharacteristic->notify();
                    Serial.print("Touch sensor ");
                    Serial.print(i + 1);
                    Serial.print(" triggered, sending code: ");
                    Serial.println(codes[i]);
                    break;
                }
            }
        } else {
            bool allReleased = true;
            for (int i = 0; i < numSensors; i++) {
                if (touchRead(touchPins[i]) < touchThreshold) {
                    allReleased = false;
                    break;
                }
            }
            if (allReleased) {
                pCharacteristic->setValue("0");
                pCharacteristic->notify();
                sensorActive = false;
                Serial.println("All sensors released.");
            }
        }

        if (!deviceConnected && oldDeviceConnected) {
            delay(500);
            pServer->startAdvertising();
            Serial.println("start advertising");
            oldDeviceConnected = deviceConnected;
        }
        if (deviceConnected && !oldDeviceConnected) {
            oldDeviceConnected = deviceConnected;
        }

        if (!deviceConnected) {
            digitalWrite(ledPin, (millis() / 500) % 2);
        }
    }

    delay(10);
}