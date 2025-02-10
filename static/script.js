async function rconExec(button) {
    const url = "/rcon_exec";
    const rcon_command = button.value
    console.log(rcon_command)
    
    const postData = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({"data":rcon_command}),
      };
    
      // Perform the fetch
      fetch(url, postData)
        .then(response => {
          if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
          }
          return response.text()
        })
        .then(data => {
          console.log('POST request successful:', data);
        })
        .catch(error => {
          console.error('Error:', error);
        });
}

addEventListener("load", (event) => {
    var titles=[
      "Welcome!", 
      "Last one alive, lock the door!", 
      "STOOOOP PLEASE", 
      "MEDIC!", 
      "Overtime!", 
      "HUDDAH HUDDAH HUH", 
      "Beep boop you are a maggot",
      "Ohhhh, I'm going to liquify ya",
      "Ohhhhhhhhhhhhhhhhh nooooooooooooooooooooooooooo, little cart is moving!",
      "WHY IS THE CART NOT MOVING?!?!?!?!",
      "I got a bucket of chicken",
      "Damnit Damnit Damnit Damnit!",
      "Nice job pardner",
      "Pornography"
    ];

    document.getElementById("title").innerHTML = titles[Math.floor(Math.random()*titles.length)];

  }
);

