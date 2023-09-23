```js
const fetch = require('node-fetch');
const MessagePack = require('what-the-pack');
const {encode, decode} = MessagePack.initialize(2 ** 22); // 4MB

fetch("http://domain.com/1/1", {
    method: "GET",
    headers: {
        "Content-Type": "application/json"
    }
}).then((response) => {
    if (!response.ok) {
        throw new Error('Network response was not ok');
    }
    return response.buffer();
})
    .then((dataBuffer) => {
        return decode(dataBuffer);
    })
    .then((resp) => {
        let data = {
            success: resp[0],
            message: resp[1],
            data: resp[2] || undefined
        }

        console.log(data)
    })
    .catch((error) => {
        console.error('Fetch error:', error);
    });
```