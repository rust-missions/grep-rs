# 멀티 쓰레드 grep

- 파일 내 문자열 검색 툴 grep을 직접 구현해본다. 
- 복수의 파일이 선택된 경우, 성능 개선을 위해 멀티 쓰레드로 동작하도록 한다.

## 🚀 기능 요구사항

기본 조건

1. 프로그램 실행 시점에 두 개의 인자(검색어, 검색 대상 경로)를 넘긴다.
2. 검색 대상이 디렉토리인 경우, 디렉토리 내 모든 파일에 대해 검사를 진행한다.
3. 해당 디렉토리 내에 또 다른 디렉토리가 존재하는 경우, 각 디렉토리 내 모든 파일에 대한 검사 또한 진행한다.
4. 멀티 스레드를 이용하여 최대한 빠르게 작업을 완료하도록 작성한다.
5. 동일한 파일에 대한 검사 결과는 한 번에 출력되어야 한다.

- Directory 내 symlink는 없다고 가정한다.
- 파일들은 모두 UTF8 인코딩으로 작성된 `.txt` 파일이라고 가정한다.

심화 조건

1. 명령어 실행 시점에 옵션을 받아 대소문자 구분 여부를 설정한다.
- 옵션을 받지 않아도 디폴트 옵션에 따라 정상 동작하도록 한다. 

2. 로컬 머신에 해당 명령어를 실제로 다운로드받아 사용할 수 있도록 한다.
- `grep-rc keyword path`와 같은 형식으로 실행될 수 있도록 한다.

## ✍🏻 입출력 요구사항

### ⌨️ 명령어 실행

프로그램 실행 시점에 두 개의 인자를 넘긴다.

- keyword : 검색 키워드
- path : 현재 위치에서의 상대 경로에 위치하는 파일 혹은 폴더

```
cargo run -- {keyword} {path}
```

### 🖥 실행 결과

keyword가 포함된 line에 대해 해당 파일, 줄 번호, 그리고 줄의 내용을 출력한다.

### 💻 실행 결과 예시: 단일 파일

```bash
$ cargo run -- body example/poem1.txt

poem1.txt 0 - I'm nobody! Who are you?
poem1.txt 1 - Are you nobody, too?
poem1.txt 5 - How dreary to be somebody!
```

### 💻 실행 결과 예시: 디렉토리

```bash
$ cargo run -- obody example

poem1.txt 0 - I'm nobody! Who are you?
poem1.txt 1 - Are you nobody, too?
poem2.txt 0 - Everybody, Somebody, Anybody, and Nobody
poem2.txt 2 - This is a little story about four people named Everybody, Somebody, Anybody, and Nobody.
poem2.txt 4 - Anybody could have done it, but Nobody did it.
poem2.txt 6 - Everybody thought that Anybody could do it, but Nobody realized that Everybody wouldn't do it.
poem2.txt 7 - It ended up that Everybody blamed Somebody when Nobody did what Anybody could have done
```

### 예시 파일

- `example/poem1.txt`

```txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

- `example/poem2.txt`

```txt
Everybody, Somebody, Anybody, and Nobody

This is a little story about four people named Everybody, Somebody, Anybody, and Nobody.
There was an important job to be done and Everybody was sure that Somebody would do it.
Anybody could have done it, but Nobody did it.
Somebody got angry about that because it was Everybody's job.
Everybody thought that Anybody could do it, but Nobody realized that Everybody wouldn't do it.
It ended up that Everybody blamed Somebody when Nobody did what Anybody could have done
```

## 🔍 진행방식

1. 미션은 [grep-rs](https://github.com/rust-missions/grep-rs) 저장소를 Fork하여 개인 저장소의 `main` 브랜치에서 진행한다.

2. 기능을 구현하기 전에 `README.md` 파일에 구현할 기능 목록을 체크리스트 형식으로 정리한다.

3. Git의 커밋 단위는 구현한 기능 단위로 추가한다.

- 커밋 메시지는 [AngularJS Commit Message Conventions](https://gist.github.com/stephenparish/9941e89d80e2bc58a153)를 참고한다.

4. 미션 구현을 완료한 후에는 자신의 Github 아이디에 해당되는 브랜치에 PR을 보낸다.

## ✔️ 과제 제출 전 체크리스트

- `cargo run`, `cargo build`를 실행했을 때 컴파일에 성공하는지 확인한다.
- 터미널에서 `cargo fmt`을 실행하여 rustfmt을 적용한다.

## 참고자료

- [Rust Book Ch12. An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Rust Book Ch16. Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- 문제 출처: [데브시스터즈 서버 직군은 왜 코딩 면접을 볼까?](https://tech.devsisters.com/posts/server-position-coding-test/)
