# [Desc]: Log Berisikan Bagaimana system pickup item dan drop items bekerja

## Masalah
### System deteksi collision dilakukan di tkphysics
ini menjadi masalah utama dari pengembangan system pickup items itu sendiri dimana niatnya saya ingin melakukan system pickup di implementasi
di tkinventory

## Main
Systems pickup disini dilakukan dengan memanfaatkan sistem event yang dimiliki oleh bevy. pertama - tama sistem physics akan melakukan pengecekan
yang mana apabila collision tersebut merupakan item maka plugin physics akan mengirimkan sinyal event yang akan diterima oleh sistem inventory
dan mulai melakukan proses insert pada inventory

tkphysics -> pengecekan collision->sinyal;
sinyal->tkinventory->operasi di tkitems;
tkitems-> pengecekan ruang dan stack, insert atau partially insert atau biarkan->despawn items yang dibawah;

semua nilai items tersebut disimpan di dalam tkinventory secara spesifik pada slot dimana juga slot amount di situ ada untuk membataskan penyimpanan
dari slot tersebut
