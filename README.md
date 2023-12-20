develoment steps:
1. create very basic tester board with serial connections, USBASP connections, and some debug LEDs. this will validate the USBasp pipeline(first test with the arduino IDE), and then enable us to test with the rust toolchain
2. in parallel, develop the schematic for the final PCB, not shipping it until phase 1. PCB has been fully validated

- phase 1 also serves as a tester platformthat can be used while debugging any hardware issues with phase 2. (essentially, instead of hardware commands, send the corresponding action over serial, this can be intercepted by a hardware visualizer on a computer to "simulate" the hardware)
