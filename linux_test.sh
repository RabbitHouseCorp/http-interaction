
echo ""
echo "Version ~> 1.0V"
echo "Is it currently compatible with Docker? No"
echo ""
echo ""
echo "🧱 Checking Go Lang Commands [1/4]"
if ! go version &> /dev/null
  then
    echo "😦 Command not found! (Go Lang is probably not installed or ENV could not be found)"
    exit
  fi
    echo "👍 Ok [2/4]"
      echo ""
        echo ""
if go build server.go 
  then 
    echo "👍 Ok [3/4]"
    echo "📦 Ready! [4/4]"
  else 
    echo "❌ There was a problem performing build."
    exit
  fi
  echo ""
  echo ""
  echo "🎉 Everything is ready. Just run \"./server and it will run automatically.\""
  echo "Thank you for downloading and using this package to extend your base."
  echo ""
  echo "If you want to help contribute, go to this project's Github."
  echo "👉 https://github.com/RabbitHouseCorp/post-interaction"

 
