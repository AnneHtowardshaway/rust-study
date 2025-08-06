# 多GitHub账号管理工具集

基于掘金文章 [《如何实现在一台电脑上同时使用多个GitHub账号》](https://juejin.cn/post/7014421400261754911) 实现的Windows PowerShell工具集，帮助开发者在一台电脑上轻松管理多个GitHub账号。

## 🚀 快速开始

### 方法一：使用主配置工具（推荐新手）

```powershell
.\multi-github-setup.ps1
```

### 方法二：快速设置（推荐熟练用户）

```powershell
.\quick-setup.ps1 -AccountName "work" -Email "work@company.com"
```

## 📁 工具列表

| 工具名称 | 功能描述 | 适用场景 |
|---------|---------|---------|
| `multi-github-setup.ps1` | 主配置工具，完全按照掘金文章方法 | 新手用户，完整配置流程 |
| `quick-setup.ps1` | 快速设置工具 | 熟练用户，快速添加账号 |
| `generate-ssh-keys.ps1` | SSH密钥生成工具 | 通用密钥生成 |
| `ssh-config-manager.ps1` | SSH配置管理工具 | 配置文件管理 |
| `copy-ssh-key.ps1` | 公钥复制工具 | 复制公钥到剪贴板 |
| `add-github-account.ps1` | GitHub账号添加工具 | 高级账号管理 |

## 📖 文档说明

| 文件名称 | 内容描述 |
|---------|---------|
| `使用说明.md` | 详细的使用指南和最佳实践 |
| `SSH密钥管理指南.md` | SSH密钥管理的完整指南 |
| `ssh-config-example.txt` | SSH配置文件示例 |

## 🔧 核心功能

### 1. 自动化配置流程
- ✅ 检查并删除全局Git配置
- ✅ 生成SSH密钥对
- ✅ 自动更新SSH配置文件
- ✅ 测试SSH连接
- ✅ 保存账号配置信息

### 2. 掘金文章方法实现
- ✅ 支持 `id_rsa_2`, `id_rsa_3` 等命名方式
- ✅ 支持 `two.github.com`, `work.github.com` 等别名
- ✅ 完全按照文章步骤执行

### 3. 多种使用方式
- ✅ 交互式菜单操作
- ✅ 命令行参数支持
- ✅ 批量配置管理

## 📋 使用示例

### 配置第二个GitHub账号

```powershell
# 运行主工具
.\multi-github-setup.ps1

# 选择"添加新的GitHub账号"
# 输入：
# - GitHub用户名: work-account
# - 邮箱: work@company.com
# - 密钥名称: id_rsa_2
# - Host别名: two.github.com
```

### 使用配置好的账号

```bash
# 克隆仓库
git clone git@two.github.com:work-account/project.git

# 进入项目目录
cd project

# 配置项目用户信息
git config user.name "Work Account"
git config user.email "work@company.com"
```

## 🔍 SSH配置示例

配置完成后，你的 `~/.ssh/config` 文件将包含：

```
# default - 默认git账户
Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_rsa

# two - 第二个账户
Host two.github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_rsa_2

# work - 工作账户
Host work.github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_rsa_work
```

## ⚡ 快速命令

```powershell
# 查看所有已配置账号
.\multi-github-setup.ps1
# 选择 "2. 查看所有已配置账号"

# 测试所有SSH连接
.\multi-github-setup.ps1
# 选择 "3. 测试所有账号SSH连接"

# 快速复制公钥
.\copy-ssh-key.ps1

# 管理SSH配置
.\ssh-config-manager.ps1
```

## 🛠️ 系统要求

- Windows 10/11
- PowerShell 5.1 或更高版本
- Git for Windows（包含SSH工具）
- 网络连接（用于测试GitHub连接）

## 📝 最佳实践

1. **删除全局Git配置**
   ```bash
   git config --global --unset user.name
   git config --global --unset user.email
   ```

2. **为每个项目单独配置用户信息**
   ```bash
   git config user.name "Your Name"
   git config user.email "your.email@example.com"
   ```

3. **使用描述性的Host别名**
   - ✅ `work.github.com`（推荐）
   - ❌ `two.github.com`（不够直观）

4. **定期测试SSH连接**
   ```bash
   ssh -T git@work.github.com
   ```

## 🐛 常见问题

### SSH连接失败
```bash
# 测试连接
ssh -T git@two.github.com -v

# 检查配置
cat ~/.ssh/config
```

### 权限被拒绝
```bash
# 检查密钥权限
ls -la ~/.ssh/

# 重新添加密钥到SSH Agent
ssh-add ~/.ssh/id_rsa_2
```

### 账号混淆
- 确保删除了全局Git配置
- 为每个项目单独设置用户信息
- 使用正确的Host别名克隆仓库

## 📚 参考资料

- [掘金原文](https://juejin.cn/post/7014421400261754911)
- [GitHub SSH文档](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)
- [Git配置文档](https://git-scm.com/book/zh/v2/自定义-Git-配置-Git)

## 🤝 贡献

欢迎提交Issue和Pull Request来改进这个工具集！

## 📄 许可证

MIT License - 详见 LICENSE 文件

---

**💡 提示：** 如果你是第一次使用，建议先阅读 `使用说明.md` 文件获取详细的使用指导。