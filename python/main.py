from flask import Flask, jsonify
import os

app = Flask(__name__)


@app.route("/")
def hello():
    return jsonify("OK")


if __name__ == "__main__":
    os.environ['WERKZEUG_RUST_MAIN'] = 'true'
    print("Running server on port 3003")
    app.run(port=3003)
