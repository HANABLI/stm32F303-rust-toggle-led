# Object dump
* Use the object dump command to check the size information of the object file. 

```cargo objdump -- -h target/thumbv7em-none-eabihf/debug/my_first_mcu_project```

* Use the read obj option to check the various defined handlers.

```cargo readobj -- -s target/thumbv7em-none-eabihf/debug/my_first_mcu_project```

* Use the read obj option to verify the text section.

```cargo readobj -- -x .text target/thumbv7em-none-eabihf/debug/my_first_mcu_project```