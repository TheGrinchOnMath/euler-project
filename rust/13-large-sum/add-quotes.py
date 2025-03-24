lines = []
with open(".\\13-large-sum\\numbers.txt", "r+") as file:
    lines = file.readlines()
    for n in range(len(lines)):
        lines[n] = '"' + lines[n][:-1] + '"\n'
with open(".\\13-large-sum\\numbers.txt", "w") as file:
    file.writelines(lines)