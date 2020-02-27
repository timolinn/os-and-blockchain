# The Linux Programming Interface

+ Unix and Linux systems have no distinction between text and binary files.
+ Low level IO syscall for Linux are _open()_,_close()_,_read()_,_write()_,_lseek()_. Standard C library counterparts are _fopen()_,_fclose()_,_fread()_,_fwrite()_,_fseek()_ and _rewind()_.
+ Linux low level io does not support User-space buffering. This feature is however supported in the standard C library.
+ There is an Over head in making a system call (This is probably the advantage of user space bufferring).
+ C standard library functions buffer in user space before making syscalls. Thereby improving performance by reducing the cost of syscalls.
+ The _inode table_ is created whenever a new file system is craeated.
+ The _inodes_ are numbered with integers. An inode hold the file's attributes which may include file permissions, Owner, group, timestamps and a pointer to the data of the file (this helps the operating system find the data of the file).
+ _Directory_ contain the actual name of the files and a hard link to the inode number.
+ Some modern linux distributions have a more dynamic data structure to thold the inode table. This means that the inode table will not be pre-created with a fixed number when a new file system is created.
+ The _stat structure_ defines and meta data of a file. We can interact with this using the `stat` syscall.
