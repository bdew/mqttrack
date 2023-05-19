# Read env vars from .env and run cargo run

switch -File .env {
  default {
    $name, $value = $_.Trim() -split '=', 2
    if ($name -and $name[0] -ne '#') {
      Set-Item "Env:$name" $value.Trim('"')
    }
  }
}

cargo run $args