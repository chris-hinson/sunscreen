import re


def hex2bin(hexc):
    return bin(int(hexc, 16))[2:].zfill(8)

def PtoFlags(P):
    return " N:"+P[0]+" V:"+P[1]+" B:"+P[2]+P[3]+" D:"+P[4]+" I:"+P[5]+" Z:"+P[6]+" C:"+P[7] + " "

#regex splits line into instruction and cpu state
p = re.compile(r"(.*)( A:.*)")
#r"(.*)( A:.*\n)"


#input file
f = open("nestest_cpu.log", "r+")
#output file
o = open("final.txt","w")


temp = f.read().splitlines()

instructions = []
states = []

for line in temp:
    #two parts of the line
    #print(line)
    instr = re.search(p,line).group(1)
    state = re.search(p,line).group(2)
    instructions.append(instr)
    states.append(state)


#print(instructions[0])
#print(states[0])

o.write("intial state (following reset vector)          ")
o.write(states[0])
o.write("\n")

for num in range(0,len(instructions)-1):
    o.write(instructions[num])
    o.write(states[num+1])
    o.write("\n")

o.write(instructions[len(instructions)-1])
#o.write(states[len(instructions)])
    
    
    



