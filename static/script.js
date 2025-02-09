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

