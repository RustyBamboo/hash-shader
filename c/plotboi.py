#!/usr/bin/env python2
import subprocess
import sys
import os
import matplotlib.pyplot as plt
import numpy as np

data = {'cuda_bitcoin':[],
        'cpu_bitcoin': []}

colors = {'cuda_bitcoin': 'red',
        'cpu_bitcoin': 'blue'}

# cuda crashes on some numbers for some reason
for blocks in [80, 400, 800, 2000, 4000, 6000, 8000, 9999, 12000, 14000, 20000, -1]:
    print blocks
    for comp in data:
        data_point = subprocess.check_output([os.path.join(os.path.dirname(__file__), comp), sys.argv[1], str(blocks)]).decode("utf-8")
        [b, t] = str(data_point).split()
        data[comp].append([int(b), float(t)])


for comp in data:
    plt.plot(np.array(data[comp])[:,0], np.array(data[comp])[:,1], c=colors[comp])
plt.show()
