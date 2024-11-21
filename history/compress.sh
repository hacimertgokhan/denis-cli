#!/bin/bash
read -p "Sıkıştırmak istediğiniz klasörün adını girin: " folder_name
if [ ! -d "$folder_name" ]; then
  echo "Hata: Belirtilen klasör bulunamadı."
  exit 1
fi
zip -r "${folder_name}.zip" "$folder_name"
tar -czf "${folder_name}.tar.gz" "$folder_name"
7z a "${folder_name}.7z" "$folder_name"
echo "Sıkıştırma işlemi tamamlandı: ${folder_name}.zip, ${folder_name}.tar.gz, ${folder_name}.7z"
