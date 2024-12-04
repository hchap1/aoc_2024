with open("../inputs/day_2.txt","r") as data: lines = [[int(y) for y in x.strip("\n").split(" ")] for x in data.readlines()]

def is_safe(readings):
    direction = 0
    for reading in readings:

count = 0

for line in lines:
    print("---")
    skipped_idx = -1    
    valid = True

    num_inc = 0
    num_dec = 0

    for i,n in enumerate(line):
        if i > 0:
            if skipped_idx != i - 1: dif = n - line[i - 1]
            else: dif = n - line[i - 2]
            if dif > 0:
                num_inc += 1
                if num_dec >= 1:
                    if skipped_idx == -1: skipped_idx = i; continue
                    else: valid = False; break
            elif dif < 0:
                num_dec += 1
                if num_inc >= 1:
                    if skipped_idx == -1: skipped_idx = i; continue
                    else: valid = False; break
            else:
                if skipped_idx == -1: skipped_idx = i; continue
                else: valid = False; break
            if abs(dif) > 3:
                if skipped_idx == -1: skipped_idx = i; continue
                else: valid = False; break

    if valid:
        count += 1

print("COUNT: %s" % count)
