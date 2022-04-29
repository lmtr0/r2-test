# Testing Cloudflare r2
Testing the s3 compatibility of cloudflare.

## Setting up

following:
```
From mr. matthew breaking changes in the cloudflare discord
Endpoint: https://{account_id}.r2.cloudflarestorage.com
Access Key:  id of the API Token
Secret Key: sha256sum of the API Token

---

Create an API token with the Account.Workers R2 Storage.Edit permission, you can also choose to allow all accounts or just one.  After creating the token (you might want to do this in a new tab or copy your api token somewhere safe first), open devtools first then go to the API Tokens list page, find the GET tokens?per_page=250 XHR request and view the response body.  Under the result array find the token you created, and copy the id (should look like a UUIDv4 without any dashes).  id will be your Access Key.

To get the sha256sum of your token, I wrote the token to a new file, .tmp (in the format CLOUDFLARE_API_TOKEN="{api_token}"), ran source .tmp, then ran echo -n "$CLOUDFLARE_API_TOKEN" | sha256sum | cut -f 1 -d " ".  This will be your Secret Key. (you may also want to also remove .tmp now and run unset CLOUDFLARE_API_TOKEN)

For anyone having problems with sha256, make sure that you are not inputting a new line after your token (that's why I used echo with the -n flag) when passing it to sha256.
``` 

create a `.env` file with:
```sh
CF_SHA256_API_KEY="SHA256_CLOUDLFARE_APIKEY"
CF_API_KEY_ID="TOKEN ID"
CF_ACCOUNT_ID="YOUR ACCOUNT ID"
```