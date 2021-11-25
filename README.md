# opcua ros2 bridge

Sets up an interface between an opc server and a pair of ros topics.

## To run

``` sh
mkdir src
cd src
git clone [this repo]
cd ..
colcon build
. install/setup.sh
ros2 run opcua_ros2_bridge opcua_ros2_bridge --ros-args -p server_address:="opc.tcp://192.168.1.33:4840/" -p node_ids:=["ns=4;i=45","ns=4;i=46","ns=4;i=47","ns=4;i=94","ns=4;i=306"]
```

Or simply
```
cargo run -- --ros-args -p server_address:="opc.tcp://192.168.1.33:4840/" -p node_ids:=["ns=4;i=45","ns=4;i=46","ns=4;i=47","ns=4;i=94","ns=4;i=306"]
```

Another example
```
cargo run -- --ros-args -p server_address:="opc.tcp://192.168.100.10:4840/" -p node_ids:=["ns=4;s=|var|CODESYS CONTROL FOR Raspberry Pi MC SL.Application.IO.bool_from_plc_1","ns=4;s=|var|CODESYS CONTROL FOR Raspberry Pi MC SL.Application.IO.bool_to_plc_1"]
```