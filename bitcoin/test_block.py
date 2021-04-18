#!/usr/bin/python3

import subprocess

import urllib.request, json 

with urllib.request.urlopen("https://blockchain.info/block-height/125552?format=json") as url:
    data = json.loads(url.read().decode())

block = data['blocks'][0]

ver = block['ver'].to_bytes(4, byteorder = 'little').hex()
prev_block = int(block['prev_block'],16).to_bytes(32, byteorder='little').hex()
mrkl_root = int(block['mrkl_root'], 16).to_bytes(32, byteorder='little').hex()
time = block['time'].to_bytes(4, byteorder='little').hex()
bits = block['bits'].to_bytes(4, byteorder='little').hex()
nonce = block['nonce'].to_bytes(4, byteorder='little').hex()

expected_hash = block['hash']

header = ver + prev_block + mrkl_root + time + bits + nonce

vulkan_runner = "../vulkan/target/release/vulkan"

# Bitcoin hash is: SHA(SHA(message))
h = subprocess.check_output(f"{vulkan_runner} {header}", shell=True)
header = h.strip().decode('utf-8')
h = subprocess.check_output(f"{vulkan_runner} {header}", shell=True)

# Reverse the endianess
final_hash = int(h.strip(), 16).to_bytes(32, byteorder="little").hex()

assert(final_hash == expected_hash)



