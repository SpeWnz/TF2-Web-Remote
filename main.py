from flask import Flask, request, render_template
import valve.rcon

app = Flask(__name__)
RCON_PASSWORD = "GROSISIMI PENNONI"
RCON_ADDRESS = ("192.168.1.241",27015)
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
    app.run(debug=True,host="0.0.0.0",port="8910")
