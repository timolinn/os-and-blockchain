/*
 * This program demostrate s the Random access using mmap
 * and native low level syscalls
 */

#include <sys/mmap.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>

struct record {
   int id;
   char name[80];
};

int main() {
    int fd;
    size_t size;
    struct record *records; // pointer to an array of records

    fd = open();
}
