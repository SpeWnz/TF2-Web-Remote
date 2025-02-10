from flask import Flask, request, render_template
import valve.rcon
import yaml

with open('config.yml', 'r') as file:
    server_conf = yaml.safe_load(file)

app = Flask(__name__)
RCON_PASSWORD = server_conf['valve']['password']
RCON_ADDRESS = (server_conf['valve']['address'],server_conf['valve']['port'])
_RCON = valve.rcon.RCON(RCON_ADDRESS,RCON_PASSWORD)

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
    
    _RCON.connect()
    _RCON.authenticate()
    app.run(debug=True,host=server_conf['web']['address'],port=server_conf['web']['port'])
