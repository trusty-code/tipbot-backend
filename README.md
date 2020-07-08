# tipbot-backend

## Endpoints

### Get QR Code Image (PNG, SVG)
[GET] /qrcodes/<iota_address>
**Returns image of QR Code**
if image is not existing, create one and safe it.


### Get user object
[GET] users/<user_uuid>


### Add address to a user
[POST] users/<user_uuid>

```
body: {
    address: "ABC"
}
```


## Deployment
.env file
```
HOST=127.0.0.1
PORT=3000
RUST_LOG=debug
```