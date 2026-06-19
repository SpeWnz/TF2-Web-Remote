from flask import Flask, request, render_template
import valve.rcon
import yaml

with open('config.yml', 'r') as file:
    server_conf = yaml.safe_load(file)

app = Flask(__name__)
RCON_PASSWORD = server_conf['valve']['password']
RCON_HOSTNAME = server_conf['valve']['address']
RCON_PORT = server_conf['valve']['port']
RCON_ADDRESS = (server_conf['valve']['address'],server_conf['valve']['port'])
_RCON = valve.rcon.RCON(RCON_ADDRESS,RCON_PASSWORD)

_debug = server_conf['system']['isDebug']

WARN = "\u001b[1;30;43m"
OKAY = "\u001b[0;30;42m"
ERR = "\u001b[1;30;41m"
RESET = "\u001b[0m"

@app.route('/')
def index():
    return render_template('index.html')

# =====================================================================================================

@app.route('/rcon_exec', methods=['POST'])
def rcon_exec():
    data = request.get_json()
    _RCON.execute(data['data'])
    
    return "OK"
    

if __name__ == '__main__':
    
    try:
        print(f"{WARN}Connecting to RCON ...{RESET}")
        _RCON.connect()
        _RCON.authenticate()
        print(f"{OKAY}Connected to RCON{RESET}")
        print(':)')
    except Exception as e:
        print(f"{ERR}Could not connect to RCON. Is the server online? Make sure it is listening on {RCON_HOSTNAME}:{RCON_PORT}{RESET}")
        print('U__U')
        exit()


    app.run(debug=_debug,host=server_conf['web']['address'],port=server_conf['web']['port'])
