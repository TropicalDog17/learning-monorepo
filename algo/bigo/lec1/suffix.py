from collections import defaultdict
if __name__ == "__main__":
    s = input()
    t = input()
    cntS, cntT = defaultdict(int, 0), defaultdict(int, 0)
    for i in range(s):
        cntS[s[i]] += 1
    for i in range(t):
        cntT[t[i]] += 1

    is_array = True
    is_automation = False
    for i in range('a', 'z'):
        if cntS[i] != cntT[i]:
            is_array = False
            continue
        if cntS[i] != 0 and cntT[i] == 0:
            is_automation = True
            continue
        if cntS[i] < cntT[i]:
            print("need tree")
            exit(0)