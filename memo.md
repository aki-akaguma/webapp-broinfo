# UserAgent

https://qiita.com/nightyknite/items/b2590a69f2e0135756dc

## Chrome

user agent reduction:
https://www.chromium.org/updates/ua-reduction/

### Desktop (user on Windows 8.1, for example)
#### old ua
```
Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.1234.56 Safari/537.36
```

#### new reducted us
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.0.0 Safari/537.36
```

### Mobile (user on Samsung Galaxy, for example)
#### old ua
```
Mozilla/5.0 (Linux; Android 9; SM-A205U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.1234.56 Mobile Safari/537.36
```

#### new reducted us
```
Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.0.0 Mobile Safari/537.36
```

### Tablet (user on Samsung Galaxy, for example)
#### old ua
```
Mozilla/5.0 (Linux; Android 9; SM-T810) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.1234.56 Safari/537.36
```

#### new reducted us
```
Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.0.0 Safari/537.36
```

## OS

### PC Windows
```
Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; WOW64; Trident/6.0)
```

### PC macOS
```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.106 Safari/537.36
```

```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:43.0) Gecko/20100101 Firefox/43.0
```

### PC Linux
```
Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0
```

```
Mozilla/5.0 (X11; Linux armv7l) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/61.0.3163.0 Safari/537.36 CrKey/1.27.96538
```

Chrome OS:
```
Mozilla/5.0 (X11; CrOS x86_64 7520.63.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.106 Safari/537.36
```

Ubuntu etc:
```
# Ubuntu
Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:24.0) Gecko/20100101 Firefox/24.0

# Mint Linux
Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.1.1) Gecko/20090716 Linux Mint/7 (Gloria) Firefox/3.5.1

# Fedora
Mozilla/5.0 (X11; U; Linux i686; fr; rv:1.9.0.10) Gecko/2009042708 Fedora/3.0.10-1.fc10 Firefox/3.0.10

# Gentoo
Mozilla/5.0 (X11; U; Linux x86_64; de; rv:1.9.0.11) Gecko/2009070611 Gentoo Firefox/3.0.11

# Arch Linux
Mozilla/5.0 (X11; Arch Linux; Linux x86_64; rv:55.0) Gecko/20100101 Firefox/55.0
```

### Other
#### BSD-OS
```
# FreeBSD
Mozilla/5.0 (X11; FreeBSD amd64; rv:41.0) Gecko/20100101 Firefox/41.0

# OpenBSD
Mozilla/5.0 (X11; U; OpenBSD amd64; en-US; rv:1.9.0.1) Gecko/2008081402 Firefox/3.0.1

# NetBSD
Mozilla/5.0 (X11; NetBSD amd64; rv:16.0) Gecko/20121102 Firefox/16.0
```

#### Game Machine (Wii/PS4/Switch/Xbox...)
```
# Wii
Mozilla/5.0 (Nintendo WiiU) AppleWebKit/536.28
(KHTML, like Gecko) NX/*** NintendoBrowser/***.US

# PlayStation 4
Mozilla/5.0 (PlayStation 4 1.52) AppleWebKit/536.26 (KHTML, like Gecko)

# PlayStation 5
Mozilla/5.0 (PlayStation 5/SmartTV) AppleWebKit/605.1.15 (KHTML, like Gecko)

Mozilla/5.0 (PlayStation 5 3.03/SmartTV) AppleWebKit/605.1.15 (KHTML, like Gecko)


# PS Vita 
Mozilla/5.0 (PlayStation Vita 1.50) AppleWebKit/531.22.8 (KHTML, like Gecko) Silk/3.2

# Nintendo Switch
Mozilla/5.0 (Nintendo Switch; ShareApplet) AppleWebKit/601.6 (KHTML, like Gecko) NF/4.0.0.5.9 NintendoBrowser/5.1.0.13341

# Comparable to the Wii U browser
Mozilla/5.0 (Nintendo WiiU) AppleWebKit/536.30 (KHTML, like Gecko) NX/3.0.4.2.11 NintendoBrowser/4.3.0.11224.US

# 3DS
Mozilla/5.0 (New Nintendo 3DS like iPhone) AppleWebKit/536.30 (KHTML, like Gecko) NX/3.0.0.5.20 Mobile NintendoBrowser/1.8.10156.US

# XBox One
Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox One) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.10553
```

#### Oculus Go
```
Mozilla/5.0 (Linux; Android 7.1.1; Pacific Build/N9F27L)AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/4.0.0.17 SamsungBrowser/4.0 Chrome/61.0.3163.109 Mobile VR Safari/537.36
```

#### Amazon Fire TV
```
# Android WebView (android.webkit.WebView)	
Mozilla/5.0 (Linux; U; Android 4.2.2; en-us; AFTB Build/JDQ39) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30

# Amazon WebView (com.amazon.android.webkit.AmazonWebView)
Mozilla/5.0 (Linux; Android 4.2.2; AFTB Build/JDQ39) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.173 Mobile Safari/537.22

# Amazon Web Appli Platform
Mozilla/5.0 (Linux; Android 4.2.2; AFTB Build/JDQ39) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.173 Mobile Safari/537.22 cordova-amazon-fireos/3.4.0 AmazonWebAppPlatform/3.4.0;2.0
```

#### Apple TV
```
AppleTV6,2/11.1

AppleTV3,1/6.0.1 (10A831)
```

## SmartPhone
### iPhone
```
Mozilla /5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B5110e Safari/601.1
```

#### Chrome on iOS
```
Mozilla/5.0 (iPhone; U; CPU iPhone OS 5_1_1 like Mac OS X; en) AppleWebKit/534.46.0 (KHTML, like Gecko) CriOS/19.0.1084.60 Mobile/9B206 Safari/7534.48.3
```

#### Safari - desktop site viewing
```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/604.1.38 (KHTML, like Gecko) Version/10.0 Safari/602.1.31
```

### Android
```
Mozilla/5.0 (Linux; U; Android 2.2.1; en-us; Nexus One Build/FRG83) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1
```

#### Android - desktop site viewing
```
Mozilla/5.0 (X11; Linux x86_64)  AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.105 Safari/537.36
```

### Windows Phone
```
Mozilla/5.0 (Windows Phone 10.0; Android 4.2.1; DEVICE INFO) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Mobile Safari/537.36 Edge/12.<OS build number>
```

```
Mozilla/5.0 (Mobile; Windows Phone 8.1; Android 4.0; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 1320) like iPhone OS 7_0_3 Mac OS X AppleWebKit/537 (KHTML, like Gecko) Mobile Safari/537
```

### Firefox OS
```
Mozilla/5.0 (Mobile; LGL25; rv:32.0) Gecko/32.0 Firefox/32.0
```

```
Mozilla/5.0 (FreeBSD; Viera; rv:34.0) Gecko/20100101 Firefox/34.0
```

### BlackBerry
```
Mozilla/5.0 (BB10; <Device Model>) AppleWebKit/<WebKit Version> (KHTML, like Gecko) Version/<BB Version #> Mobile Safari/<WebKit Version>

Mozilla/5.0 (BlackBerry; U; BlackBerry 9320; en-GB) AppleWebKit/534.11+ (KHTML, like Gecko) Version/7.1.0.398 Mobile Safari/534.11+

Mozilla/5.0 (Linux; Android 8.1.0; BBF100-6 Build/OPM1.171019.019) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.158 Mobile Safari/537.36
```

## Tablet
### iPad
```
Mozilla/5.0 (iPad; CPU OS 10_0_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/14A403 Safari/602.1
```

### Android Tablet
```
Mozilla/5.0 (Linux; Android 5.0.2; SM-T530 Build/LRX22G) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.83 Safari/537.36
```

### Kindle Fire OS
```
Mozilla/5.0 (Linux; U; Android 2.3.4; en-us; Kindle Fire Build/GINGERBREAD) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1
```

```
Mozilla/5.0 (Linux; U; Android 2.3.4; en-us; Silk/1.0.146.3-Gen4_12000410) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1 Silk-Accelerated=true
```

### Windows Tablet
```
Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko
```

```
Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; ARM; Trident/6.0; Touch)
```

```
Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; .NET4.0E; .NET4.0C; .NET CLR 3.5.30729; .NET CLR 2.0.50727; .NET CLR 3.0.30729; Tablet PC 2.0; GWX:QUALIFIED; rv:11.0) like Gecko
```

## Feature Phone
### docomo
```
DoCoMo/2.0 F2051(c100;TB;serXXXXXXXXXXXXXXX;iccxxxxxxxxxxxxxxxxxxxx)
```

### au
```
KDDI-[機種名] UP.Browser/[ブラウザバージョン] (GUI) [サーバ名]/[サーババージョン]
```

### SoftBank
```
Vodafone/1.0/V904SH/SHJ001/SN  Browser/VF-NetFront/3.3 Profile/MIDP-2.0 Configuration/CLDC-1.1
```

```
SoftBank/1.0/910T/TJ001/SN Browser/NetFront/3.3 Profile/MIDP-2.0 Configuration/CLDC-1.1
```

## Browser
### Internet Explorer
before IE10:
```
Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)
```

after IE 11:
```
Mozilla/5.0 (Windows NT 6.3; Win64; x64; Trident/7.0; rv:11.0) like Gecko
```

### Microsoft Edge
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.<OS build number>
```

#### Microsoft Edge based on chromium
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.48 Safari/537.36 Edg/74.1.96.24
```

```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3794.0 Safari/537.36 Edg/76.0.161.0
```

### Chrome
```
Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/45.0.2454.99 Safari/537.36
```

### Firefox
```
Mozilla/5.0 (platform; rv:geckoversion) Gecko/20100101 Firefox/firefoxversion
```

### Safari
```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/600.8.9 (KHTML, like Gecko) Version/8.0.8 Safari/600.8.9
```

### Opera
```
Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/45.0.2454.85 Safari/537.36 OPR/32.0.1948.25
```

old version before based on chromium:
```
Opera/9.80 (Windows NT 6.1; WOW64) Presto/2.12.388 Version/12.18
```

### Vivaldi
```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.88 Safari/537.36 Vivaldi/2.4.1488.36
```

after vivaldi 2.10:
```
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.94 Safari/537.36
```

### SRware Iron
```
Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3250.0 Iron Safari/537.36
```

### Sleipnir
```
Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/36.0.1985.143 Safari/537.36 Sleipnir/6.1.0
```

### Xiaomi MIUI Browser
```
Mozilla/5.0 (Linux; U; Android 7.1.2; zh-cn; MI 5C Build/N2G47J) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/53.0.2785.146 Mobile Safari/537.36 XiaoMi/MiuiBrowser/9.1.3
```

### Brave
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36
```

### Samsung Internet
```
Mozilla/5.0 (Linux; Android 5.0.2; SAMSUNG SM-G925F Build/LRX22G) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/4.0 Chrome/44.0.2403.133 Mobile Safari/537.36
```

### UCBrowser
```
Mozilla/5.0 (Linux; U; Android 5.1; en-US; Aqua Joy Build/LMY47D) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 UCBrowser/11.2.5.932 U3/0.8.0 Mobile Safari/534.30
```

