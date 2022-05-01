import re


def hex2bin(hexc):
    return bin(int(hexc, 16))[2:].zfill(8)

def PtoFlags(P):
    return " N:"+P[0]+" V:"+P[1]+" B:"+P[2]+P[3]+" D:"+P[4]+" I:"+P[5]+" Z:"+P[6]+" C:"+P[7] + " "


p = re.compile(r" P:([0-9a-fA-F][0-9a-fA-F])")

f = open("nestest-redux.log", "r+")
o = open("final.txt","w")


temp = f.read().splitlines()

for line in temp:
    #new_line = re.sub(p,hex2bin('\g<1>'),line)
    old_str = re.search(p,line).group(1)
    new_str = hex2bin(old_str)
    flags = PtoFlags(new_str)
    new_line = re.sub(p," P: "+flags,line)
    o.write(new_line)
    o.write("\n")



