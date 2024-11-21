# DenisDB CLI

DenisDB, bellek tabanlı ve protobuff destekli veritabanı yönetimi sağlayan bir veritabanı dilidir. Bu CLI aracılığıyla DenisDB'yi yönetebilir ve çeşitli veritabanı işlemlerini gerçekleştirebilirsiniz.

## Komutlar / Commands

### Genel Bilgi / General Info

- `--version`:  
  **Türkçe:** Sürüm bilgilerini gösterir.  
  **English:** Displays version information.

- `--about`:  
  **Türkçe:** Veritabanı hakkında bilgi verir.  
  **English:** Provides information about the database.

### Bellek Kullanımı / Memory Usage

- `--mu`:  
  **Türkçe:** DenisDB'nin bellek kullanımını izlersiniz.  
  **English:** Monitors the memory usage of DenisDB.

- `--lm`:  
  **Türkçe:** DenisDB'nin bellek kullanımını süreç yapısı şeklinde izlersiniz.  
  **English:** Monitors the memory usage of DenisDB in a process structure.

### Token Yönetimi / Token Management

- `--token`:  
  **Türkçe:** Veri tokenleri kullanımı.  
  **English:** Token usage management.
    - `-l`:  
      **Türkçe:** Token listesini önizler.  
      **English:** Previews the list of tokens.
    - `-c`:  
      **Türkçe:** Yeni bir token (veri bölümü) oluşturur.  
      **English:** Creates a new token (data section).
    - `-i`:  
      **Türkçe:** Tokenler (veri bölümleri) hakkında bilgi verir.  
      **English:** Provides information about tokens (data sections).

### Konfigürasyon / Configuration

- `--opt`:  
  **Türkçe:** DenisDB'nin ayarlarını ve kullanıcı tercihlerini hızlıca değiştirir.  
  **English:** Quickly changes DenisDB settings and user preferences.
    - `-lang:`  
      **Türkçe:** Dil seçeneklerini belirler.  
      **English:** Changes language settings.
        - `-slfs <tr,en>`:  
          **Türkçe:** Sadece aktif oturum boyunca dil değişikliği yapar.  
          **English:** Changes the language for the current session only.
        - `-slfg <tr,en>`:  
          **Türkçe:** Tüm oturumlar için kalıcı bir dil değişikliği yapar.  
          **English:** Changes the language permanently for all sessions.
    - `-smts <Sayı>`:  
      **Türkçe:** Maksimum oluşturulabilir token (veri bölümü) miktarını ayarlar.  
      **English:** Sets the maximum number of tokens (data sections) that can be created.
    - `-ctea <t/f>`:  
      **Türkçe:** Seçilen token için her girişte yeni token sistemini aktif/pasif yapar.  
      **English:** Activates/deactivates the new token system on every entry for the selected token.
    - `-umte <t/f>`:  
      **Türkçe:** Genel token sistemini aktif/pasif yapar. (Not: Bu seçenek aktif edildiğinde ana token ile tüm projelere erişim sağlanabilir.)  
      **English:** Activates/deactivates the global token system. (Note: When activated, the main token grants access to all projects.)

### Çıkış / Exit

- `--exit`:  
  **Türkçe:** Manuel moddan çıkış yapar.  
  **English:** Exits the manual mode.

---

## Manuel Değişiklikler / Manual Changes

**Türkçe:**  
Yukarıdaki komutlar ile yapılan tüm değişiklikler manuel olarak da yapılabilir, işlemler `denis.properties` dosyasına kaydedilir.

**English:**  
All changes made using the above commands can also be done manually, and the operations are saved in the `denis.properties` file.
