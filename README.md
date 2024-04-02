sudo ln -s /opt/homebrew/opt/mysql@8.0/lib/libmysqlclient.21.dylib /usr/local/mysql/lib/libmysqlclient.dylib

RUSTFLAGS="-L/opt/homebrew/opt/mysql-client@8.0/lib" cargo install diesel_cli
