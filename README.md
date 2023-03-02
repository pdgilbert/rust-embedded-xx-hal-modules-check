# rust-embedded-xx-hal-modules-check
Check common structure in xxx_hal for various MCUs.

This code uses a crate-like structure. Building the crate for different HALs reults in a check that various
peripherals are in common locations and common syntax is used. This will not work for everything because 
different MCUs need some different things. But the hope is that as much as possible can be made similar. 
Results can be most easily seen under 'Actions' at the github site 
'https://github.com/pdgilbert/rust-embedded-xx-hal-modules-check/actions'

The purpose of these tests (examples) is strictly to see if they compile. 
There is nothing here that would be interesting to load. 
See 'https://github.com/pdgilbert/rust-integration-testing' for real examples that can be loaded,
and the 'Actions' in that repository to see if they compile for different MCUs.

## Building

The examples can be built manually by setting one of these lines:
```
               environment variables for cargo                    
  _____________________________________________________________   
  export HAL=stm32f0xx MCU=stm32f030xc TARGET=thumbv6m-none-eabi  
  export HAL=stm32f1xx MCU=stm32f103   TARGET=thumbv7m-none-eabi  
  export HAL=stm32f3xx MCU=stm32f303xc TARGET=thumbv7em-none-eabih
  export HAL=stm32f4xx MCU=stm32f401   TARGET=thumbv7em-none-eabih
  export HAL=stm32f7xx MCU=stm32f769   TARGET=thumbv7em-none-eabih
  export HAL=stm32g0xx MCU=stm32g081   TARGET=thumbv6m-none-eabi  
  export HAL=stm32g4xx MCU=stm32g473   TARGET=thumbv7em-none-eabih
  export HAL=stm32h7xx MCU=stm32h742   TARGET=thumbv7em-none-eabih
  export HAL=stm32l0xx MCU=stm32l0x2kztx TARGET=thumbv6m-none-eabi 72KZTx
  export HAL=stm32l1xx MCU=stm32l100   TARGET=thumbv7m-none-eabi  
  export HAL=stm32l1xx MCU=stm32l151   TARGET=thumbv7m-none-eabi  
  export HAL=stm32l4xx MCU=stm32l422   TARGET=thumbv7em-none-eabi 

```
then to build
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example xxx
```
where `xxx` is replaced by one of the example names, such as
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example level-0
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example level-3
```
Check files in directory `examples/` to see what is attempted in the build. The purpose of theseThere is no 
