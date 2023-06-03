# Batmanager
Tool for configuring Lenovo battery settings. Based on `battmgr`.

### Usage
```
Usage: batmanager [OPTIONS]

Options:
  -c, --conservation [<on/off>]  get or set the conservation mode [possible values: on, off]
  -r, --rapid [<on/off>]         get or set the rapid charing mode [possible values: on, off]
  -p, --performance [<1/2/3>]    get or set the performance mode [possible values: "intelligent-cooling (1)", "extreme-performance (2)", "battery-saving (3)"]
  -h, --help                     Print help
  -V, --version                  Print version
  ```

#### Examples
Read current configuration: `batmanager`  
Read performance mode: `batmanager -p`  
Turn on conservation mode: `batmanager -c on`  
Turn off rapid charge mode: `batmanager -r off`  
Turn conservation mode off, rapid charge on and set performance mode to "Intelligent Cooling" : `batmanager -c0 -r1 -p1`  

### Installation
#### Cargo
```
cargo install batmanager
```
#### ArchLinux
```
paru -S batmanager
```
