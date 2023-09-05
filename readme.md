# prime number calc by rust lang

러스트로 만든 소수 구하기 프로그램 입니다.

multi thread와 single thread 버전이 있습니다.

go 언어로 만들었던 https://github.com/kasworld/primenum 의 rust 버전 입니다.

최고 성능을 위해서는 work thread count를 logical cpu count -1 로 하세요. 
( main thread가 cpu를 하나 사용합니다. )