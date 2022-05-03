CLOUDFLARE_API_TOKEN=$1
echo "SecretKey:"
echo -n $CLOUDFLARE_API_TOKEN | sha256sum |  cut -f 1 -d " " 