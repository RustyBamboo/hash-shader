# Flip endians of hash
import csv


f2 = open("./block_data_flip.csv", "w")
with open("./block_data.csv") as csvfile:
    reader = csv.reader(csvfile)
    for row in reader:
        header = row[0]
        expected = row[1]
        flip_hash = int(expected, 16).to_bytes(32, byteorder="little").hex()
        f2.write(f"{header},{flip_hash}\n")

f2.close()


