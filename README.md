
# wifi2dcom
# Use a D-COM as if it was a WifiCom

Simply plug in your D-COM, and start the application providing the serial port and the configs present in yout secrets.py file given by the wificom.dev website
If you don't want to pass in every configuration of the secrets.py file in the command line, you can store it in a JSON file and use that instead, like so:
```json
{
	"username" : "USERNAME",
	"password" : "PASSWORD",
	"user_uuid": "USER_UUID",
	"device_uuid": "DEVICE_UUID"
}
```

# Why did make this instead of using a wificom like a normal person??
## Because i'm too lazy (and incompetent) to make a wificom!

If you are into vpets, especially the digimon ones, you must have heard about the D-COM, it's a really awesome tool that permits us to communicate with our devices and copy their communication code so we can share with other people around the world! Even awesome than that is a wificom, which essentially does the same thing but instead of depending on a serial device and whatnot, you use the power of IoT to achieve the same thing! (And some bonuses, but the important stuff for this repo is this).

The thing is, i have a DCOM, but not an wificom,  and i am a negation when it comes to anything relating to hardware, so building an wificom seems really way out of my league, i am however a software guy, and since wificom does everything with APIs and whatnot, why not make an application to simulate an wificom and communicate with my arduino to achieve the same result? (The fact that w0rld on linux does not work with and d-com natively may have convinced me to start this, but that's beside the point 🤭🤭)

Oh the magic of doing simple stuff the more complicated, but cool (or actually stupid), way! So yeah, if you had the need for stuff like this like i had, there you go!

