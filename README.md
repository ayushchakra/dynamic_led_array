develoment steps:
1. create very basic tester board with serial connections, USBASP connections, and some debug LEDs. this will validate the USBasp pipeline(first test with the arduino IDE), and then enable us to test with the rust toolchain - also might be nice to include a small version of the programmable led array
2. in parallel, develop the schematic for the final PCB, not shipping it until phase 1. PCB has been fully validated

- phase 1 also serves as a tester platformthat can be used while debugging any hardware issues with phase 2. (essentially, instead of hardware commands, send the corresponding action over serial, this can be intercepted by a hardware visualizer on a computer to "simulate" the hardware

- phase 1 can use the LED Driver eval board (just one) with one LED array to serve as a testing platform, for future iterations, the chip can be integrated into the carrier board

LED array options:
|Link	|Communication Protocol	|Price|Daisy Chain?|
|-------|-----------------------|-----|------------|
[16x16](https://www.amazon.com/BTF-LIGHTING-Individual-Addressable-Flexible-Controllers/dp/B088BTYJH6/ref=asc_df_B088BTYJH6/?tag=hyprod-20&linkCode=df0&hvadid=647362028828&hvpos=&hvnetw=g&hvrand=5458167644642167462&hvpone=&hvptwo=&hvqmt=&hvdev=c&hvdvcmdl=&hvlocint=&hvlocphy=9003506&hvtargid=pla-1007368042799&mcid=40403d086ed33ef990504ed588694c3e&th=1)| Unknown | 18.99|
[9x16](https://www.digikey.com/en/products/detail/adafruit-industries-llc/2947/5959341)| I2C | 

- [LED matrix](https://www.digikey.com/en/products/detail/adafruit-industries-llc/2973/5959350)
- [LED driver eval board](https://www.digikey.com/en/products/detail/adafruit-industries-llc/2946/5959340?utm_adgroup=&utm_source=google&utm_medium=cpc&utm_campaign=Pmax_Shopping_Boston%20Metro%20Category%20Awarness&utm_term=&utm_content=&utm_id=go_cmp-20837509568_adg-_ad-__dev-c_ext-_prd-5959340_sig-Cj0KCQiAm4WsBhCiARIsAEJIEzU_WlsF7jv6vVdvdmGJQAAfPQx7lVcIfZWQJUs7w_8XBYPrdWWNItYaAvw1EALw_wcB&gad_source=1&gclid=Cj0KCQiAm4WsBhCiARIsAEJIEzU_WlsF7jv6vVdvdmGJQAAfPQx7lVcIfZWQJUs7w_8XBYPrdWWNItYaAvw1EALw_wcB)
- [LED driver chip](https://www.digikey.com/en/products/detail/lumissil-microsystems/IS31FL3731-QFLS2-TR/4286473)
