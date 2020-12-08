# Indoor GPS - Real World MicroControllers

A centimeter-accurate position tracking solution for Olin College's Real World Microcontroller course.


The Real-Time-Location-System (RTLS) in this project was designed using KiCad, manufactured by JLCPCB, and assembled by myself (SMD by hand!). My firmware tools of choice were the Rust programing language for firmware , Probe-rs and JLink for flashing and debugging, and, again, Rust for the visualization and web integration. The localization was done in-firmware using a novel and extremely power efficient 1-way-ranging algorithm.



## Packages
----

| Package         | Purpose                                                   |
| --------------- | --------------------------------------------------------- |
| Docs            | Documentation of the indoor gps project                   |
| blinky          | Kicad files for a very simple blinky project to learn fab |
| nrf-tests       | Testing the deployment of rust code onto the nrf-52840    |
| dmw1000-carrier | Kicad files for a carrier board for the dwm1001           |
| nrf-accel       | a crate that provides access to the ___ accelerometer     |
