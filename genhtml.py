import ZHOR_Modules.fileManager as fm
import sys

file = str(sys.argv[1])
#print(file)

lines = fm.fileToSimpleList(file)

for l in lines:
    #print(l)
    msg = f"#{l}\n"
    msg += "{\n"
    msg += f"\tbackground:radial-gradient(var(--buttonbg), "
    msg += f"rgba(0, 0, 0, 0)), url('assets/images/ctf/{l}.jpg');\n"
    msg += "}\n"
    print(msg)
