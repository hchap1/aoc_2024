with open("..\\inputs\\day_1.txt", "r") as data: lines = [x.strip("\n").split("  ") for x in data.readlines()]
a, b, c = ([int(x[0]) for x in lines],[int(x[1]) for x in lines],{})
a.sort()
b.sort()
dif, sim = (0,0)
for i in range(len(a)):
    dif += abs(a[i] - b[i])
    if c.get(b[i]) == None: c[b[i]] = 1
    else: c[b[i]] += 1
for n in a: sim += n * c[n] if c.get(n) != None else 0
print(f"P1: {dif}\nP2: {sim}")
