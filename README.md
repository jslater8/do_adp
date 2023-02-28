Josh's incredible adp downloader thingy

Instructions

Build the programs

cargo build

1. Login to the adp website
2. Click "View statement" (NOT View all Statements)
3. Run josh_point_finder and hover over the next button (on the bottom left) and the download button (on the top right) note down the coordinates.
e.g.

```
cargo run --bin josh_point_finder
```

4. Run josh_downloader with the next button and the download button coordinates as the first 4 arguments, then the number of statements to download and optionally the delay between clicking (to account for different internet speeds, defaults to 5 seconds).
e.g. The following command runs the josh_downloader with next button coordinates at (1582, 221) and download button coordinates at (2400, 210) for 53 files. (This example comes from the edge browser at 1440p resolution).

```
cargo run --bin josh_downloader -- 1583 221 2400 210 53
```

NOTE: You can't use your computer whilst josh_downloader is running otherwise all sorts of shit will be clicked and it won't be happy.

5. This should download the files to your normal download folder. Please double check that the correct number of files is there and there are no duplicates if there are maybe change the default click delay. This will take a decent amount of time because adp is slow as hell to load the statements.