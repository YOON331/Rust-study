# Rush Hour Puzzle
- [Rush Hour Puzzle Assignment 출처](https://github.com/hongshin/LearningC/tree/master/rushhour)   
- 이 프로그램은 Rush Hour Puzzle을 커맨드라인 인터페이스에서 실행할 수 있는 기능을 제공합니다.
- Rush Hour Puzzle은 6x6 으로 구성되어 있으며, 1번의 자동차가 F4 위치하면 게임은 종료됩니다.


## Features
**게임 화면을 출력하는 함수 `display`**   
  - Array2D<usize>의 puzzle을 순회하며 출력합니다.   
  <br/>   

**게임을 실행하기 위해 초기값을 설정하는 함수`load_game`**   
  - input_arr의 정보를 읽어와서 자동차의 정보를 저장한 Vec\<car>를 반환합니다.   
  <br/>   

**퍼즐을 업데이트 하는 함수 `update_puzzle`**   
  - 입력받은 자동차 정보를 바탕으로 puzzle의 정보를 업데이트합니다.   
  <br/>   

**자동차를 움직일 수 있는지 확인하는 함수 `move_car`**   
  - 입력받은 커맨드라인에 따라 자동차를 움직일 수 있는지 확인합니다.   
  <br/>   

**F4에 1번 자동차가 위치해 있는지 확인하는 함수 `check_destination`**   
  - 해당 퍼즐의 위치에 1번 자동차가 있는지 확인하여 boolean 타입을 반환합니다.   


## 실행하기
**1. git clone**
  ~~~
  git clone https://github.com/YOON331/Rust-study.git
  ~~~
<br/>

**2. 폴더 이동 및 실행**
  ~~~
  cd exercises/rush_hour
  ~~~
  ~~~
  cargo run
  ~~~
<br/>

**3. 커맨드라인 입력하면서 게임 즐기기**   
- 커맨드라인 설명   
`start` : 게임 실행 또는 실행중인 게임 초기화    
`quit`: 게임 종료   
`right <자동차번호>` 오른쪽으로 1칸 이동   
`left <자동차번호>` 왼쪽으로 1칸 이동   
`up <자동차번호>` 위쪽으로 1칸 이동   
`down <자동차번호>` 아래쪽으로 1칸 이동   
