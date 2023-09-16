// # go-cqhttp 默认配置文件
//
// account: # 账号相关
//   uin: 1233456 # QQ账号
//   password: '' # 密码为空时使用扫码登录
//   encrypt: false  # 是否开启密码加密
//   status: 0      # 在线状态 请参考 https://docs.go-cqhttp.org/guide/config.html#在线状态
//   relogin: # 重连设置
//     delay: 3   # 首次重连延迟, 单位秒
//     interval: 3   # 重连间隔
//     max-times: 0  # 最大重连次数, 0为无限制
//
//   # 是否使用服务器下发的新地址进行重连
//   # 注意, 此设置可能导致在海外服务器上连接情况更差
//   use-sso-address: true
//   # 是否允许发送临时会话消息
//   allow-temp-session: false
//
//
//   # 数据包的签名服务器
//   # 兼容 https://github.com/fuqiuluo/unidbg-fetch-qsign
//   # 暂不支持最新版qsign服务可用（v1.1.0）
//   # 如果遇到 登录 45 错误, 或者发送信息风控的话需要填入一个服务器
//   # 示例:
//   # sign-server: 'http://127.0.0.1:8080' # 本地签名服务器
//   # sign-server: 'https://signserver.example.com' # 线上签名服务器
//   # 服务器可使用docker在本地搭建或者使用他人开放的服务
//   # 不建议使用公共服务器, 有封号风险
//   sign-server: '-'
//
// heartbeat:
//   # 心跳频率, 单位秒
//   # -1 为关闭心跳
//   interval: 5
//
// message:
//   # 上报数据类型
//   # 可选: string,array
//   post-format: string
//   # 是否忽略无效的CQ码, 如果为假将原样发送
//   ignore-invalid-cqcode: false
//   # 是否强制分片发送消息
//   # 分片发送将会带来更快的速度
//   # 但是兼容性会有些问题
//   force-fragment: false
//   # 是否将url分片发送
//   fix-url: false
//   # 下载图片等请求网络代理
//   proxy-rewrite: ''
//   # 是否上报自身消息
//   report-self-message: false
//   # 移除服务端的Reply附带的At
//   remove-reply-at: false
//   # 为Reply附加更多信息
//   extra-reply-data: false
//   # 跳过 Mime 扫描, 忽略错误数据
//   skip-mime-scan: false
//
// output:
//   # 日志等级 trace,debug,info,warn,error
//   log-level: warn
//   # 日志时效 单位天. 超过这个时间之前的日志将会被自动删除. 设置为 0 表示永久保留.
//   log-aging: 15
//   # 是否在每次启动时强制创建全新的文件储存日志. 为 false 的情况下将会在上次启动时创建的日志文件续写
//   log-force-new: true
//   # 是否启用日志颜色
//   log-colorful: true
//   # 是否启用 DEBUG
//   debug: false # 开启调试模式
//
// # 默认中间件锚点
// default-middlewares: &default
//   # 访问密钥, 强烈推荐在公网的服务器设置
//   access-token: ''
//   # 事件过滤器文件目录
//   filter: ''
//   # API限速设置
//   # 该设置为全局生效
//   # 原 cqhttp 虽然启用了 rate_limit 后缀, 但是基本没插件适配
//   # 目前该限速设置为令牌桶算法, 请参考:
//   # https://baike.baidu.com/item/%E4%BB%A4%E7%89%8C%E6%A1%B6%E7%AE%97%E6%B3%95/6597000?fr=aladdin
//   rate-limit:
//     enabled: false # 是否启用限速
//     frequency: 1  # 令牌回复频率, 单位秒
//     bucket: 1     # 令牌桶大小
//
// database: # 数据库相关设置
//   leveldb:
//     # 是否启用内置leveldb数据库
//     # 启用将会增加10-20MB的内存占用和一定的磁盘空间
//     # 关闭将无法使用 撤回 回复 get_msg 等上下文相关功能
//     enable: true
//
//   # 媒体文件缓存， 删除此项则使用缓存文件(旧版行为)
//   cache:
//     image: data/image.db
//     video: data/video.db
//
// # 连接服务列表
// servers:
//   # 添加方式，同一连接方式可添加多个，具体配置说明请查看文档
//   #- http: # http 通信
//   #- ws:   # 正向 Websocket
//   #- ws-reverse: # 反向 Websocket
//   #- pprof: #性能分析服务器
//   - http: # HTTP 通信设置
//       address: 0.0.0.0:5700 # HTTP监听地址
//       timeout: 5      # 反向 HTTP 超时时间, 单位秒，<5 时将被忽略
//       long-polling:   # 长轮询拓展
//         enabled: false       # 是否开启
//         max-queue-size: 2000 # 消息队列大小，0 表示不限制队列大小，谨慎使用
//       middlewares:
//         <<: *default # 引用默认中间件
//       post:           # 反向HTTP POST地址列表
//       #- url: ''                # 地址
//       #  secret: ''             # 密钥
//       #  max-retries: 3         # 最大重试，0 时禁用
//       #  retries-interval: 1500 # 重试时间，单位毫秒，0 时立即
//       #- url: http://127.0.0.1:5701/ # 地址
//       #  secret: ''                  # 密钥
//       #  max-retries: 10             # 最大重试，0 时禁用
//       #  retries-interval: 1000      # 重试时间，单位毫秒，0 时立即
//
//   # LambdaServer 配置
//   - lambda:
//       type: scf # scf: 腾讯云函数 aws: aws Lambda
//       middlewares:
//         <<: *default # 引用默认中间件
//
//   # 正向WS设置
//   - ws:
//       # 正向WS服务器监听地址
//       address: 0.0.0.0:8080
//       middlewares:
//         <<: *default # 引用默认中间件
//
//   # 反向WS设置
//   - ws-reverse:
//       # 反向WS Universal 地址
//       # 注意 设置了此项地址后下面两项将会被忽略
//       universal: ws://your_websocket_universal.server
//       # 反向WS API 地址
//       api: ws://your_websocket_api.server
//       # 反向WS Event 地址
//       event: ws://your_websocket_event.server
//       # 重连间隔 单位毫秒
//       reconnect-interval: 3000
//       middlewares:
//         <<: *default # 引用默认中间件
//
//   # 可添加更多
//   #- ws-reverse:
//   #- ws:
//   #- http:

// struct Config {
//     account: Account,
//     heartbeat: Heartbeat,
//     message: Message,
//     output: Output,
//     default_middlewares: DefaultMiddlewares,
//     database: Database,
//     servers: Vec<Server>,
// }
//
// struct Account {
//     /// QQ账号
//     uin: i64,
//     /// QQ密码，密码为空时使用扫码登录
//     password: String,
//     encrypt: bool,
//     status: i64,
//     relogin: ReLogin,
//     use_sso_address: bool,
//     allow_temp_session: bool,
//     sign_server: String,
// }
//
// struct ReLogin {
//     delay: i64,
//     interval: i64,
//     max_times: i64,
// }
