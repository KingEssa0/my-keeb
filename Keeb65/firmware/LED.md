# Caps Lock LED

Use one diffused red 5 mm LED and a 1 kΩ resistor. The LED stays on while the computer reports Caps Lock on and goes dark when Caps Lock is off. There are no animations.

With USB unplugged, wire Pico header pin **29 (GP22)** through the **1 kΩ resistor** to the LED anode. Connect the LED cathode to Pico header pin **28 (GND)**. Insulate the exposed connections and secure the LED where it is visible. Pin numbers here are the physical Pico header positions.

Flash [Keeb65.uf2](Keeb65.uf2) and press Caps Lock twice to check both states.

[LED part](https://www.adafruit.com/product/299) · [Pico pinout](https://pip-assets.raspberrypi.com/categories/610-raspberry-pi-pico/documents/RP-008307-DS-2-pico-datasheet.pdf)
