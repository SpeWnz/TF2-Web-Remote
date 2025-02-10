async function rconExec(button) {
  const url = "/rcon_exec";
  const rcon_command = button.value
  console.log(rcon_command)

  const postData = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ "data": rcon_command }),
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

  let loc = window.location.hash.split('#')[1];
  
  if (loc == undefined) {
    loc = "mapchange";
  }

  goTo(loc);

  titling();
}
);

function titling() {
  var titles = [
    "Welcome!",
    "Last one alive, lock the door!",
    "STOOOOP PLEASE",
    "MEDIC!",
    "Overtime!",
    "HUDDAH HUDDAH HUH",
    "Beep boop you are a maggot",
    "Ohhhh, I'm going to liquify ya",
    "Little cart is moving!",
    "WHY IS THE CART NOT MOVING?!?!?!?!",
    "I got a bucket of chicken",
    "Damnit Damnit Damnit Damnit!",
    "Nice job pardner",
    "Pornography"
  ];

  document.getElementById("title").innerHTML = titles[Math.floor(Math.random() * titles.length)];
}

function goTo(loc) {
  var sectors = document.getElementsByTagName("main");

  for (i = 0; i < sectors.length; i++) {
    let elem = sectors[i];

    if (elem.getAttribute("id") != loc) {
      console.log(elem.getAttribute("id"));
      elem.style.display = "none";
    }
    else { elem.style.display = "flex"; }
  }
}

