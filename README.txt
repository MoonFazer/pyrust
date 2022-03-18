To deploy a rust module into python:

 - build the module
 - cargo build --release
 - cp path/to/executable.so executable.so
 - import functions in the python!