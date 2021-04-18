#!/usr/bin/env python
import subprocess
import os
import sys
import csv

if len(sys.argv) != 2:
    print "Exactly 1 arg required: block_data.csv"
    exit(-1)

with open(sys.argv[1], 'r') as bitcoin:
    bc_reader = csv.reader(bitcoin)
    data = ' '
    hashes = ' '
    for row in bc_reader:
        data += row[0] + ' '
        hashes += row[1] + ' '
    pipe = subprocess.Popen(os.path.join(os.path.dirname(__file__), 'cuda_bitcoin') + data, shell=True,
                stdout=subprocess.PIPE)
    out = pipe.communicate()[0]

    out = out.split()
    hashes = hashes.split()
    for i in range(len(out)):
        print out[i] == hashes[i]


