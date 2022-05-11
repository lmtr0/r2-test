# Testing Cloudflare r2
Testing the s3 compatibility of cloudflare.

## Setting up

following:

From mr. matthew breaking changes in the cloudflare discord <br>
| name | value |
| --- | --- | 
| Endpoint | `https://{account_id}.r2.cloudflarestorage.com` |
| Access Key | id of the API Token |
| Secret Key | sha256sum of the API Token |

Create an API token with the Account.Workers R2 Storage.Edit permission, you can also choose to allow all accounts or just one.  <br>
After creating the token (you might want to do this in a new tab or copy your api token somewhere safe first), <br>
open browsers devtools first then go to the API Tokens list page, find the **`GET` https://dash.cloudflare.com/api/v4/user/tokens?per_page=250** XHR request and view the response body.  <br>
Under the result array find the token you created, and copy the id (should look like a UUIDv4 without any dashes).  id will be your Access Key. <br>
```json
{
	"result": [
		{
			"id": "The Id",
			"name": "Token name",
		}
	]
    // rest of the request
}
```

To get the sha256sum of your token:
```sh
CLOUDFLARE_API_TOKEN="YOUR API TOKEN"
echo -n "$CLOUDFLARE_API_TOKEN" | sha256sum | cut -f 1 -d " "
```


create a `.env` file with:
```sh
SECRET_KEY="SHA256_CLOUDLFARE_APIKEY"
ACCESS_KEY="TOKEN ID"
CF_ACCOUNT_ID="YOUR ACCOUNT ID"
```