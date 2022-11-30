<#
.SYNOPSIS
    Runs all days sequentially

#>

Get-ChildItem `
    | Where-Object Name -match "day" `
    | Sort-Object -Property {
        [void]($_ -match "day_(\d+)")
        [int]$Matches.1
    } `
    | ForEach-Object {
        Write-Host "$($_.Name)" -ForegroundColor Yellow
        cargo run --release --quiet -p $_.Name
        Write-Host ""
    }

