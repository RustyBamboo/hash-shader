#!/usr/bin/python3


import urllib.request, json

blocks = range(201290, 300000)


for b in blocks:
    print("Downloading block %s"%b)
    f = open("block_data_200000_300000.csv", "a")
    with urllib.request.urlopen("https://blockchain.info/block-height/%s?format=json"%b) as url:
        data = json.loads(url.read().decode())

    block = data['blocks'][0]

    ver = block['ver'].to_bytes(4, byteorder = 'little').hex()
    prev_block = int(block['prev_block'],16).to_bytes(32, byteorder='little').hex()
    mrkl_root = int(block['mrkl_root'], 16).to_bytes(32, byteorder='little').hex()
    time = block['time'].to_bytes(4, byteorder='little').hex()
    bits = block['bits'].to_bytes(4, byteorder='little').hex()
    nonce = block['nonce'].to_bytes(4, byteorder='little').hex()
    
    expected_hash = block['hash']
    flip_hash = int(expected_hash, 16).to_bytes(32, byteorder="little").hex()
    
    header = ver + prev_block + mrkl_root + time + bits + nonce     

    f.write("%s,%s\n"%(header, flip_hash))
    f.close() # so that is actually saves


        
