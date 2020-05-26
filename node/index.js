const express = require("express");

const app = express();

app.get("/", (req, res) => {
  res.json("OK")
})

app.listen(3000, (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("Server running on port 3000");
  }
});
