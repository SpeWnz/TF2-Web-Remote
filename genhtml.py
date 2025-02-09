import ZHOR_Modules.fileManager as fm
import sys

file = str(sys.argv[1])


lines = fm.fileToSimpleList(file)

#<button id="pl_thundermountain" value="changelevel pl_thundermountain" onclick="rconExec(this); return false;">pl_thundermountain</button>

for l in lines:
    html = f'<button id="{l}" value="changelevel {l}" onclick="rconExec(this); return false;">{l}</button>'
    print(html)