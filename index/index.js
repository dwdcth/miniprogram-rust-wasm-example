import { sum, hello_world ,md5} from '../rust-mod/pkg/rust_mod'

Page({
  data: { },
  onLoad: function () {
    console.log(md5("xxx"));
    this.setData({
      hello: hello_world(),
      sum: sum(1, 2)
    })
  },
})
