#!/bin/bash

for i in {1..1000} 
    do 
    echo "2";
    ./asmtest < 100 > test.in
    ./asm < test.in > testasm.out
    cs241.binasm < test.in ? testbinasm.out
    diff testasm.out testbinasm.out
    done